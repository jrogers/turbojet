// Command peer runs one quickfix-go session, driven by turbojet-interop's tests over stdin and
// stdout. It speaks the same protocol as the QuickFIX/J peer (../peer, documented on its Peer
// class), so the same scenarios run against both engines.
//
// Arguments are key=value: role, begin, port, sender, target, sender-sub, target-sub,
// target-location, heartbeat, reset-on-logon, reconnect, max-latency, log-dir, as for the
// QuickFIX/J peer, and spec-dir: the directory holding quickfix-go's data dictionaries. For TLS,
// tls-ca, tls-cert, tls-key and need-client-auth, as for the QuickFIX/J peer: PEM files, which
// quickfix-go reads itself.
//
// quickfix-go logs its errors and its other session events through the same call, so all of
// them are emitted as qfgo_event; there is no qfj_error.
package main

import (
	"bufio"
	"crypto/tls"
	"fmt"
	"io"
	"net"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"github.com/quickfixgo/quickfix"
	filelog "github.com/quickfixgo/quickfix/log/file"
)

const soh = "\x01"

var (
	emitLock sync.Mutex
	// The highest MsgSeqNum passed to FromAdmin or FromApp.
	lastDelivered atomic.Int64
)

func emit(kind, payload string) {
	emitLock.Lock()
	defer emitLock.Unlock()
	fmt.Printf("%s\t%s\n", kind, payload)
}

// printable writes SOH as |, except inside XmlData(213), which is as long as XmlDataLen(212) says
// and may contain SOH itself: there it's written \x01.
func printable(raw string) string {
	if at := strings.Index(raw, soh+"212="); at >= 0 {
		digits := at + len(soh+"212=")
		if end := strings.Index(raw[digits:], soh); end >= 0 {
			end += digits
			length, err := strconv.Atoi(raw[digits:end])
			start := end + 1 + len("213=")
			if err == nil && strings.HasPrefix(raw[end+1:], "213=") && start+length <= len(raw) {
				data := strings.ReplaceAll(raw[start:start+length], soh, `\x01`)
				raw = raw[:start] + data + raw[start+length:]
			}
		}
	}
	return strings.ReplaceAll(raw, soh, "|")
}

func main() {
	opts := map[string]string{}
	for _, arg := range os.Args[1:] {
		key, value, _ := strings.Cut(arg, "=")
		opts[key] = value
	}
	get := func(key, fallback string) string {
		if v, ok := opts[key]; ok {
			return v
		}
		return fallback
	}
	role, begin := opts["role"], opts["begin"]

	session := quickfix.NewSessionSettings()
	set := session.Set
	set("BeginString", begin)
	set("SenderCompID", opts["sender"])
	set("TargetCompID", opts["target"])
	if v := opts["sender-sub"]; v != "" {
		set("SenderSubID", v)
	}
	if v := opts["target-sub"]; v != "" {
		set("TargetSubID", v)
	}
	if v := opts["target-location"]; v != "" {
		set("TargetLocationID", v)
	}
	set("HeartBtInt", get("heartbeat", "30"))
	set("ResetOnLogon", get("reset-on-logon", "N"))
	set("ReconnectInterval", get("reconnect", "1"))
	set("CheckLatency", "Y")
	set("MaxLatency", get("max-latency", "120"))
	// quickfix-go sessions are always on unless given a schedule, like NonStopSession=Y.
	spec := opts["spec-dir"]
	if begin == "FIXT.1.1" {
		set("DefaultApplVerID", "FIX.5.0SP2")
		set("TransportDataDictionary", filepath.Join(spec, "FIXT11.xml"))
		set("AppDataDictionary", filepath.Join(spec, "FIX50SP2.xml"))
	} else {
		// FIX.4.2 -> FIX42.xml
		set("DataDictionary", filepath.Join(spec, strings.ReplaceAll(begin, ".", "")+".xml"))
	}

	link := &link{}
	settings := quickfix.NewSettings()
	settings.GlobalSettings().Set("FileLogPath", opts["log-dir"])
	var port int
	var listener net.Listener
	if role == "acceptor" {
		// Any free port on loopback. quickfix-go only accepts a session on the port configured for
		// it, so bind first, configure the port bound, and hand it this listener.
		var err error
		listener, err = net.Listen("tcp", "127.0.0.1:0")
		check(err)
		port = listener.Addr().(*net.TCPAddr).Port
		settings.GlobalSettings().Set("SocketAcceptHost", "127.0.0.1")
		set("SocketAcceptPort", strconv.Itoa(port))
	} else {
		// Through a relay of the peer's own, which the disconnect command closes: quickfix-go has
		// no call that drops a connection without a Logout.
		var err error
		port, err = strconv.Atoi(opts["port"])
		check(err)
		relay, err := link.relay(port)
		check(err)
		set("SocketConnectHost", "127.0.0.1")
		set("SocketConnectPort", strconv.Itoa(relay))
	}
	if ca, ok := opts["tls-ca"]; ok {
		// quickfix-go reads an acceptor's TLS settings from the global settings, an initiator's
		// from its session's.
		tlsSettings := session
		if role == "acceptor" {
			tlsSettings = settings.GlobalSettings()
		}
		configureTLS(tlsSettings, ca, opts)
	}
	id, err := settings.AddSession(session)
	check(err)

	logs, err := filelog.NewLogFactory(settings)
	check(err)
	store := quickfix.NewMemoryStoreFactory()
	var connector interface {
		Start() error
		Stop()
	}
	if role == "acceptor" {
		acceptor, err := quickfix.NewAcceptor(app{}, store, settings, emittingLogs{logs})
		check(err)
		acceptor.SetNewListenerCallback(func(_ string, config *tls.Config) (net.Listener, error) {
			// Tracked beneath TLS, so disconnect closes the TCP connection.
			if config != nil {
				return tls.NewListener(link.track(listener), config), nil
			}
			return link.track(listener), nil
		})
		connector = acceptor
		check(connector.Start())
	} else {
		initiator, err := quickfix.NewInitiator(app{}, store, settings, emittingLogs{logs})
		check(err)
		connector = initiator
		check(connector.Start())
	}
	emit("ready", strconv.Itoa(port))

	var stop sync.Once
	stopConnector := func() { stop.Do(connector.Stop) }
	in := bufio.NewScanner(os.Stdin)
	in.Buffer(make([]byte, 1<<20), 1<<24)
	for in.Scan() {
		command, arg, _ := strings.Cut(in.Text(), " ")
		if command == "quit" {
			break
		}
		if err := run(id, link, stopConnector, command, arg); err != nil {
			emit("error", command+": "+err.Error())
		} else {
			emit("ok", command)
		}
	}
	stopConnector()
	os.Exit(0)
}

// configureTLS sets quickfix-go's TLS settings. SocketUseSSL=Y means TLS without requiring a
// client certificate: without it, an acceptor with a certificate requires and verifies one, and an
// initiator without a certificate doesn't use TLS at all.
func configureTLS(settings *quickfix.SessionSettings, ca string, opts map[string]string) {
	settings.Set("SocketCAFile", ca)
	if cert, ok := opts["tls-cert"]; ok {
		settings.Set("SocketCertificateFile", cert)
		settings.Set("SocketPrivateKeyFile", opts["tls-key"])
	}
	if opts["need-client-auth"] != "Y" {
		settings.Set("SocketUseSSL", "Y")
	}
}

func check(err error) {
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func run(id quickfix.SessionID, link *link, stop func(), command, arg string) error {
	switch command {
	case "send":
		return send(id, arg)
	case "send-many":
		count, fields, _ := strings.Cut(arg, " ")
		n, err := strconv.Atoi(count)
		if err != nil {
			return err
		}
		for i := 0; i < n; i++ {
			if err := send(id, strings.ReplaceAll(fields, "{i}", strconv.Itoa(i))); err != nil {
				return err
			}
		}
		return nil
	case "set-next-sender-seq":
		n, err := strconv.Atoi(arg)
		if err != nil {
			return err
		}
		return quickfix.SetNextSenderMsgSeqNum(id, n)
	case "set-next-target-seq":
		n, err := strconv.Atoi(arg)
		if err != nil {
			return err
		}
		if err := awaitProcessed(id); err != nil {
			return err
		}
		return quickfix.SetNextTargetMsgSeqNum(id, n)
	case "sequence-reset":
		// Not atomic, as for the QuickFIX/J peer: scenarios using it keep HeartBtInt long.
		if err := send(id, "35=4|123=N|36="+arg); err != nil {
			return err
		}
		n, err := strconv.Atoi(arg)
		if err != nil {
			return err
		}
		return quickfix.SetNextSenderMsgSeqNum(id, n)
	case "test-request":
		return send(id, "35=1|112="+arg)
	case "logout":
		// quickfix-go logs a session out only by stopping it; Stop waits for the Logout reply.
		go stop()
		return nil
	case "disconnect":
		return link.drop()
	}
	return fmt.Errorf("unknown command")
}

// awaitProcessed waits until quickfix-go has finished with the last message it delivered: like
// QuickFIX/J, it advances its expected MsgSeqNum only after FromAdmin or FromApp returns.
func awaitProcessed(id quickfix.SessionID) error {
	deadline := time.Now().Add(5 * time.Second)
	for {
		expected, err := quickfix.GetExpectedTargetNum(id)
		if err != nil {
			return err
		}
		if int64(expected) > lastDelivered.Load() {
			return nil
		}
		if time.Now().After(deadline) {
			return fmt.Errorf("still processing MsgSeqNum %d", lastDelivered.Load())
		}
		time.Sleep(time.Millisecond)
	}
}

// send builds a message from tag=value|...: 35 (and XmlData) in the header, the rest in the body.
func send(id quickfix.SessionID, fields string) error {
	msg := quickfix.NewMessage()
	for _, field := range strings.Split(fields, "|") {
		tag, value, _ := strings.Cut(field, "=")
		n, err := strconv.Atoi(tag)
		if err != nil {
			return err
		}
		value = strings.ReplaceAll(value, `\x01`, soh)
		if n == 35 || n == 212 || n == 213 {
			msg.Header.SetString(quickfix.Tag(n), value)
		} else {
			msg.Body.SetString(quickfix.Tag(n), value)
		}
	}
	return quickfix.SendToTarget(msg, id)
}

func delivered(msg *quickfix.Message) {
	seq, err := msg.Header.GetInt(34)
	if err != nil {
		return
	}
	for {
		last := lastDelivered.Load()
		if int64(seq) <= last || lastDelivered.CompareAndSwap(last, int64(seq)) {
			return
		}
	}
}

type app struct{}

func (app) OnCreate(quickfix.SessionID) {}
func (app) OnLogon(quickfix.SessionID)  { emit("logon", "") }
func (app) OnLogout(quickfix.SessionID) { emit("logout", "") }
func (app) ToAdmin(msg *quickfix.Message, _ quickfix.SessionID) {
	emit("to_admin", printable(msg.String()))
}
func (app) ToApp(msg *quickfix.Message, _ quickfix.SessionID) error {
	emit("to_app", printable(msg.String()))
	return nil
}
func (app) FromAdmin(msg *quickfix.Message, _ quickfix.SessionID) quickfix.MessageRejectError {
	delivered(msg)
	emit("from_admin", printable(msg.String()))
	return nil
}
func (app) FromApp(msg *quickfix.Message, _ quickfix.SessionID) quickfix.MessageRejectError {
	delivered(msg)
	emit("from_app", printable(msg.String()))
	return nil
}

// emittingLogs logs to files and also emits the wire traffic and session events.
type emittingLogs struct{ inner quickfix.LogFactory }

func (f emittingLogs) Create() (quickfix.Log, error) {
	file, err := f.inner.Create()
	return emittingLog{file}, err
}
func (f emittingLogs) CreateSessionLog(id quickfix.SessionID) (quickfix.Log, error) {
	file, err := f.inner.CreateSessionLog(id)
	return emittingLog{file}, err
}

type emittingLog struct{ file quickfix.Log }

func (l emittingLog) OnIncoming(msg []byte) {
	l.file.OnIncoming(msg)
	emit("in", printable(string(msg)))
}
func (l emittingLog) OnOutgoing(msg []byte) {
	l.file.OnOutgoing(msg)
	emit("out", printable(string(msg)))
}
func (l emittingLog) OnEvent(text string) {
	l.file.OnEvent(text)
	emit("qfgo_event", strings.NewReplacer(soh, "|", "\n", " ").Replace(text))
}
func (l emittingLog) OnEventf(format string, args ...any) { l.OnEvent(fmt.Sprintf(format, args...)) }

// link holds the session's current connection, so disconnect can close it.
type link struct {
	mu    sync.Mutex
	conns []net.Conn
}

func (l *link) set(conns ...net.Conn) {
	l.mu.Lock()
	defer l.mu.Unlock()
	l.conns = conns
}

func (l *link) drop() error {
	l.mu.Lock()
	defer l.mu.Unlock()
	if len(l.conns) == 0 {
		return fmt.Errorf("not connected")
	}
	for _, conn := range l.conns {
		conn.Close()
	}
	l.conns = nil
	return nil
}

// track records each connection the acceptor accepts.
func (l *link) track(listener net.Listener) net.Listener { return trackingListener{listener, l} }

type trackingListener struct {
	net.Listener
	link *link
}

func (t trackingListener) Accept() (net.Conn, error) {
	conn, err := t.Listener.Accept()
	if err == nil {
		t.link.set(conn)
	}
	return conn, err
}

// relay listens on a loopback port of its own and forwards each connection quickfix-go makes to
// it on to port, returning the relay's port.
func (l *link) relay(port int) (int, error) {
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return 0, err
	}
	go func() {
		for {
			inner, err := listener.Accept()
			if err != nil {
				return
			}
			outer, err := net.Dial("tcp", net.JoinHostPort("127.0.0.1", strconv.Itoa(port)))
			if err != nil {
				inner.Close()
				continue
			}
			l.set(inner, outer)
			go pipe(inner, outer)
			go pipe(outer, inner)
		}
	}()
	return listener.Addr().(*net.TCPAddr).Port, nil
}

// pipe copies until either side closes, then closes both, so a close is passed on.
func pipe(to, from net.Conn) {
	io.Copy(to, from)
	to.Close()
	from.Close()
}
