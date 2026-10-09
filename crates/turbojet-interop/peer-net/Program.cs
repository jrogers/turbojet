// One QuickFIX/n session, driven by turbojet-interop's tests over stdin and stdout. It speaks the
// same protocol as the QuickFIX/J peer (../peer, documented on its Peer class), so the same
// scenarios run against all three engines.
//
// Arguments are key=value: role, begin, port, sender, target, sender-sub, target-sub,
// sender-location, target-location, heartbeat, reset-on-logon, reconnect, max-latency and log-dir,
// as for the QuickFIX/J peer, and spec-dir: the directory holding QuickFIX/n's data dictionaries
// (by default, spec/ next to this program). For TLS, tls-ca and need-client-auth as for the
// QuickFIX/J peer, and, for a certificate to present, tls-bundle and tls-bundle-password: a PKCS#12
// bundle, which is how QuickFIX/n takes one.
//
// QuickFIX/n logs its errors and its other session events through the same call, so all of them
// are emitted as qfn_event; there is no qfj_error.

using System.Net;
using System.Net.Sockets;
using System.Text;
using QuickFix;
using QuickFix.Fields;
using QuickFix.Logger;
using QuickFix.Store;
using QuickFix.Transport;

const char Soh = '\u0001';

var opts = new Dictionary<string, string>();
foreach (var arg in args)
{
    var eq = arg.IndexOf('=');
    opts[arg[..eq]] = arg[(eq + 1)..];
}
string Get(string key, string fallback) => opts.TryGetValue(key, out var value) ? value : fallback;
var role = opts["role"];
var begin = opts["begin"];

var id = new SessionID(
    begin,
    opts["sender"], Get("sender-sub", ""), Get("sender-location", ""),
    opts["target"], Get("target-sub", ""), Get("target-location", ""));

var session = new SettingsDictionary();
session.SetString("ConnectionType", role);
session.SetString("BeginString", begin);
session.SetString("SenderCompID", opts["sender"]);
session.SetString("TargetCompID", opts["target"]);
foreach (var (option, setting) in new[]
{
    ("sender-sub", "SenderSubID"), ("target-sub", "TargetSubID"),
    ("sender-location", "SenderLocationID"), ("target-location", "TargetLocationID"),
})
{
    if (Get(option, "") is var value && value != "")
    {
        session.SetString(setting, value);
    }
}
session.SetString("NonStopSession", "Y");
session.SetString("HeartBtInt", Get("heartbeat", "30"));
session.SetString("ResetOnLogon", Get("reset-on-logon", "N"));
session.SetString("CheckLatency", "Y");
session.SetString("MaxLatency", Get("max-latency", "120"));
session.SetString("UseDataDictionary", "Y");
var spec = Get("spec-dir", Path.Combine(AppContext.BaseDirectory, "spec"));
if (begin == "FIXT.1.1")
{
    session.SetString("DefaultApplVerID", "FIX.5.0SP2");
    session.SetString("TransportDataDictionary", Path.Combine(spec, "FIXT11.xml"));
    session.SetString("AppDataDictionary", Path.Combine(spec, "FIX50SP2.xml"));
}
else
{
    // FIX.4.2 -> FIX42.xml
    session.SetString("DataDictionary", Path.Combine(spec, begin.Replace(".", "") + ".xml"));
}

if (opts.TryGetValue("tls-ca", out var ca))
{
    session.SetString("SSLEnable", "Y");
    session.SetString("SSLCACertificate", ca);
    // By default it checks revocation online, which certificates without a CRL distribution
    // point fail. As with the other peers, only Turbojet checks revocation, against CRLs given.
    session.SetString("SSLCheckCertificateRevocation", "N");
    if (opts.TryGetValue("tls-bundle", out var bundle))
    {
        session.SetString("SSLCertificate", bundle);
        session.SetString("SSLCertificatePassword", Get("tls-bundle-password", ""));
    }
    if (role == "acceptor")
    {
        // An acceptor that validates certificates refuses a client without one, whatever
        // SSLRequireClientCertificate says, so it validates only when it requires one.
        var clientAuth = Get("need-client-auth", "N");
        session.SetString("SSLRequireClientCertificate", clientAuth);
        session.SetString("SSLValidateCertificates", clientAuth);
    }
    else
    {
        // The name the server's certificate must carry, as Turbojet's client checks it.
        session.SetString("SSLValidateCertificates", "Y");
        session.SetString("SSLServerName", "localhost");
    }
}

int port;
if (role == "acceptor")
{
    // QuickFIX/n can't report the port an acceptor bound (GetAcceptorAddresses isn't implemented),
    // so take a free one on loopback first. Another process could take it in between; in a test
    // run on one machine that's unlikely enough.
    var probe = new TcpListener(IPAddress.Loopback, 0);
    probe.Start();
    port = ((IPEndPoint)probe.LocalEndpoint).Port;
    probe.Stop();
    session.SetString("SocketAcceptHost", "127.0.0.1");
    session.SetString("SocketAcceptPort", port.ToString());
}
else
{
    port = int.Parse(opts["port"]);
    session.SetString("SocketConnectHost", "127.0.0.1");
    session.SetString("SocketConnectPort", port.ToString());
}

var settings = new SessionSettings();
// QuickFIX/n reads these from the defaults, not the session's settings: the log path for the log
// of events outside a session, and an initiator's ReconnectInterval (30 s if it's not there).
var defaults = new SettingsDictionary();
defaults.SetString("FileLogPath", opts["log-dir"]);
defaults.SetString("ReconnectInterval", Get("reconnect", "1"));
settings.Set(defaults);
settings.Set(id, session);
var app = new PeerApp();
var store = new MemoryStoreFactory();
var logs = new EmittingLogFactory(new FileLogFactory(settings));
IInitiator? initiator = null;
IAcceptor? acceptor = null;
if (role == "acceptor")
{
    acceptor = new ThreadedSocketAcceptor(app, store, settings, logs);
    acceptor.Start();
}
else
{
    initiator = new SocketInitiator(app, store, settings, logs);
    initiator.Start();
}
Events.Emit("ready", port.ToString());

string? line;
while ((line = Console.In.ReadLine()) != null)
{
    var space = line.IndexOf(' ');
    var command = space < 0 ? line : line[..space];
    var argument = space < 0 ? "" : line[(space + 1)..];
    if (command == "quit")
    {
        break;
    }
    try
    {
        Run(Session.LookupSession(id) ?? throw new InvalidOperationException("no session"), command, argument);
        Events.Emit("ok", command);
    }
    catch (Exception e)
    {
        Events.Emit("error", $"{command}: {e.Message}");
    }
}
initiator?.Stop(true);
acceptor?.Stop(true);
Environment.Exit(0);

void Run(Session session, string command, string argument)
{
    switch (command)
    {
        case "send":
            Send(session, argument);
            break;
        case "send-many":
            var space = argument.IndexOf(' ');
            var count = int.Parse(argument[..space]);
            var fields = argument[(space + 1)..];
            for (var i = 0; i < count; i++)
            {
                Send(session, fields.Replace("{i}", i.ToString()));
            }
            break;
        case "set-next-sender-seq":
            session.NextSenderMsgSeqNum = ulong.Parse(argument);
            break;
        case "set-next-target-seq":
            AwaitProcessed(session);
            session.NextTargetMsgSeqNum = ulong.Parse(argument);
            break;
        case "sequence-reset":
            // Not atomic, as for the QuickFIX/J peer: scenarios using it keep HeartBtInt long.
            Send(session, "35=4|123=N|36=" + argument);
            session.NextSenderMsgSeqNum = ulong.Parse(argument);
            break;
        case "test-request":
            Send(session, "35=1|112=" + argument);
            break;
        case "logout":
            session.Logout();
            break;
        case "disconnect":
            session.Disconnect("disconnect command");
            break;
        default:
            throw new ArgumentException("unknown command");
    }
}

// Waits until QuickFIX/n has finished with the last message it delivered: like QuickFIX/J, it
// advances its expected MsgSeqNum only after FromAdmin or FromApp returns.
void AwaitProcessed(Session session)
{
    var deadline = DateTime.UtcNow.AddSeconds(5);
    while (session.NextTargetMsgSeqNum <= Events.LastDelivered)
    {
        if (DateTime.UtcNow > deadline)
        {
            throw new InvalidOperationException($"still processing MsgSeqNum {Events.LastDelivered}");
        }
        Thread.Sleep(1);
    }
}

// A message from tag=value|...: 35 (and XmlData) in the header, the rest in the body.
void Send(Session session, string fields)
{
    var msg = new Message();
    foreach (var field in fields.Split('|'))
    {
        var eq = field.IndexOf('=');
        var tag = int.Parse(field[..eq]);
        var value = field[(eq + 1)..].Replace("\\x01", Soh.ToString());
        if (tag is 35 or 212 or 213)
        {
            msg.Header.SetField(new StringField(tag, value));
        }
        else
        {
            msg.SetField(new StringField(tag, value));
        }
    }
    if (!session.Send(msg))
    {
        throw new InvalidOperationException("not sent: session not logged on");
    }
}

static class Events
{
    private static readonly object Lock = new();
    private static long _lastDelivered;

    // The highest MsgSeqNum passed to FromAdmin or FromApp.
    public static ulong LastDelivered => (ulong)Interlocked.Read(ref _lastDelivered);

    public static void Emit(string kind, string payload)
    {
        lock (Lock)
        {
            Console.Out.Write($"{kind}\t{payload}\n");
            Console.Out.Flush();
        }
    }

    public static void Delivered(Message msg)
    {
        if (!msg.Header.IsSetField(Tags.MsgSeqNum))
        {
            return;
        }
        var seq = (long)msg.Header.GetULong(Tags.MsgSeqNum);
        long last;
        while (seq > (last = Interlocked.Read(ref _lastDelivered)))
        {
            if (Interlocked.CompareExchange(ref _lastDelivered, seq, last) == last)
            {
                return;
            }
        }
    }

    // The message with | for SOH, except inside XmlData(213), which is as long as XmlDataLen(212)
    // says and may contain SOH itself: there it's written \x01.
    public static string Printable(string raw)
    {
        var length = "\u0001212=";
        var at = raw.IndexOf(length, StringComparison.Ordinal);
        if (at >= 0)
        {
            var digits = at + length.Length;
            var end = raw.IndexOf('\u0001', digits);
            if (end >= 0 && int.TryParse(raw.AsSpan(digits, end - digits), out var size))
            {
                var start = end + 1 + "213=".Length;
                if (raw.AsSpan(end + 1).StartsWith("213=") && start + size <= raw.Length)
                {
                    var data = raw.Substring(start, size).Replace("\u0001", "\\x01");
                    raw = raw[..start] + data + raw[(start + size)..];
                }
            }
        }
        return raw.Replace('\u0001', '|');
    }
}

sealed class PeerApp : IApplication
{
    public void OnCreate(SessionID id) { }
    public void OnLogon(SessionID id) => Events.Emit("logon", "");
    public void OnLogout(SessionID id) => Events.Emit("logout", "");
    public void ToAdmin(Message msg, SessionID id) => Events.Emit("to_admin", Events.Printable(msg.ToString()));
    public void ToApp(Message msg, SessionID id) => Events.Emit("to_app", Events.Printable(msg.ToString()));

    public void FromAdmin(Message msg, SessionID id)
    {
        Events.Delivered(msg);
        Events.Emit("from_admin", Events.Printable(msg.ToString()));
    }

    public void FromApp(Message msg, SessionID id)
    {
        Events.Delivered(msg);
        Events.Emit("from_app", Events.Printable(msg.ToString()));
    }
}

// Logs to files and also emits the wire traffic and session events.
sealed class EmittingLogFactory(ILogFactory inner) : ILogFactory
{
    public ILog Create(SessionID id) => new EmittingLog(inner.Create(id));
    public ILog CreateNonSessionLog() => new EmittingLog(inner.CreateNonSessionLog());
}

sealed class EmittingLog(ILog file) : ILog
{
    public void Clear() => file.Clear();

    public void OnIncoming(string msg)
    {
        file.OnIncoming(msg);
        Events.Emit("in", Events.Printable(msg));
    }

    public void OnOutgoing(string msg)
    {
        file.OnOutgoing(msg);
        Events.Emit("out", Events.Printable(msg));
    }

    public void OnEvent(string text)
    {
        file.OnEvent(text);
        Events.Emit("qfn_event", text.Replace('\u0001', '|').Replace('\n', ' ').Replace('\r', ' '));
    }

    public void Dispose() => file.Dispose();
}
