package dev.turbojet.interop;

import java.io.BufferedReader;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.GeneralSecurityException;
import java.security.KeyFactory;
import java.security.KeyStore;
import java.security.PrivateKey;
import java.security.cert.Certificate;
import java.security.cert.CertificateFactory;
import java.security.spec.PKCS8EncodedKeySpec;
import java.util.Base64;
import java.util.HashMap;
import java.util.Map;

import quickfix.Application;
import quickfix.Connector;
import quickfix.DefaultMessageFactory;
import quickfix.FieldNotFound;
import quickfix.FileLogFactory;
import quickfix.Log;
import quickfix.LogFactory;
import quickfix.MemoryStoreFactory;
import quickfix.Message;
import quickfix.Session;
import quickfix.SessionID;
import quickfix.SessionSettings;
import quickfix.SocketAcceptor;
import quickfix.SocketInitiator;
import quickfix.field.MsgSeqNum;

/**
 * One QuickFIX/J session, driven by turbojet-interop's tests.
 *
 * <p>Arguments are {@code key=value}: role (acceptor|initiator), begin (FIX.4.2|FIX.4.3|FIX.4.4|FIXT.1.1),
 * port (initiator only; the acceptor picks a free one and reports it), sender, target,
 * heartbeat (seconds), reset-on-logon (Y|N), reconnect (seconds), max-latency (seconds: how far
 * SendingTime may be from the clock, with CheckLatency=Y), log-dir. For TLS: tls-ca (PEM: the
 * CA trusted for the other side's certificate), tls-cert and tls-key (PEM: the certificate to
 * present, and its PKCS#8 key; none, and none is presented), need-client-auth (Y|N, as acceptor).
 *
 * <p>Commands, one per line on stdin, each answered with {@code ok} or {@code error}:
 * <ul>
 *   <li>{@code send <fields>}: sends a message; fields are {@code tag=value} separated by
 *       {@code |}, including 35; QuickFIX/J adds the rest of the header. SOH inside a value is
 *       written {@code \x01}.</li>
 *   <li>{@code send-many N <fields>}: sends N messages, each with {@code {i}} in the fields
 *       replaced by its index from 0, without waiting for the counterparty.</li>
 *   <li>{@code set-next-sender-seq N}, {@code set-next-target-seq N}: change sequence numbers
 *       without telling the counterparty.</li>
 *   <li>{@code sequence-reset N}: sends SequenceReset-Reset with NewSeqNo N, then sends from N.</li>
 *   <li>{@code test-request ID}, {@code logout}, {@code disconnect} (no Logout), {@code quit}.</li>
 * </ul>
 *
 * <p>Events go to stdout, one per line, as {@code kind TAB payload}: {@code ready <port>},
 * {@code logon}, {@code logout}, {@code from_admin}/{@code from_app}/{@code to_admin}/
 * {@code to_app} with the message ({@code |} for SOH, and {@code \x01} for SOH inside
 * XmlData(213)), {@code ok <command>} and {@code error <text>}. From QuickFIX/J's log:
 * {@code in}/{@code out} with the bytes on the wire (written the same way), and
 * {@code qfj_error <text>}.
 */
public final class Peer implements Application {
    private static final char SOH = '\u0001';

    /** The highest MsgSeqNum passed to fromAdmin or fromApp. */
    private static volatile int lastDelivered;

    private static synchronized void emit(String kind, String payload) {
        System.out.println(kind + "\t" + payload);
        System.out.flush();
    }

    private static String wire(Message msg) {
        return printable(msg.toString());
    }

    /**
     * The message with {@code |} for SOH, except inside XmlData(213), which is as long as
     * XmlDataLen(212) says and may contain SOH itself: there it's written {@code \x01}.
     */
    private static String printable(String raw) {
        String length = SOH + "212=";
        int at = raw.indexOf(length);
        if (at >= 0) {
            int digits = at + length.length();
            int end = raw.indexOf(SOH, digits);
            try {
                int start = end + 1 + "213=".length();
                int stop = start + Integer.parseInt(raw.substring(digits, end));
                if (end >= 0 && raw.startsWith("213=", end + 1) && stop <= raw.length()) {
                    String data = raw.substring(start, stop).replace(String.valueOf(SOH), "\\x01");
                    raw = raw.substring(0, start) + data + raw.substring(stop);
                }
            } catch (NumberFormatException | StringIndexOutOfBoundsException e) {
                // Not a well-formed XmlData: print it like any other field.
            }
        }
        return raw.replace(SOH, '|');
    }

    public static void main(String[] args) throws Exception {
        Map<String, String> opts = new HashMap<>();
        for (String arg : args) {
            int eq = arg.indexOf('=');
            opts.put(arg.substring(0, eq), arg.substring(eq + 1));
        }
        String role = opts.get("role");
        String begin = opts.get("begin");
        SessionID id = new SessionID(begin,
                opts.get("sender"), opts.getOrDefault("sender-sub", ""), opts.getOrDefault("sender-location", ""),
                opts.get("target"), opts.getOrDefault("target-sub", ""), opts.getOrDefault("target-location", ""),
                "");

        SessionSettings settings = new SessionSettings();
        settings.setString(id, "ConnectionType", role);
        settings.setString(id, "NonStopSession", "Y");
        settings.setString(id, "HeartBtInt", opts.getOrDefault("heartbeat", "30"));
        settings.setString(id, "ResetOnLogon", opts.getOrDefault("reset-on-logon", "N"));
        settings.setString(id, "ReconnectInterval", opts.getOrDefault("reconnect", "1"));
        settings.setString(id, "CheckLatency", "Y");
        settings.setString(id, "MaxLatency", opts.getOrDefault("max-latency", "120"));
        settings.setString(id, "FileLogPath", opts.get("log-dir"));
        settings.setString(id, "UseDataDictionary", "Y");
        if (begin.equals("FIXT.1.1")) {
            settings.setString(id, "DefaultApplVerID", "FIX.5.0SP2");
            settings.setString(id, "TransportDataDictionary", "FIXT11.xml");
            settings.setString(id, "AppDataDictionary", "FIX50SP2.xml");
        } else {
            // FIX.4.2 -> FIX42.xml
            settings.setString(id, "DataDictionary", begin.replace(".", "") + ".xml");
        }
        if (opts.containsKey("tls-ca")) {
            configureTls(settings, id, opts, role);
        }
        int port = 0;
        if (role.equals("acceptor")) {
            // Any free port; read back once bound. On loopback, not the wildcard address: on macOS
            // a wildcard bind can get a port that some process (it was the Gradle daemon) already
            // listens on at 127.0.0.1, and that socket then takes the connections meant for this one.
            settings.setString(id, "SocketAcceptAddress", "127.0.0.1");
            settings.setString(id, "SocketAcceptPort", "0");
        } else {
            port = Integer.parseInt(opts.get("port"));
            settings.setString(id, "SocketConnectHost", "127.0.0.1");
            settings.setString(id, "SocketConnectPort", Integer.toString(port));
        }

        Peer app = new Peer();
        MemoryStoreFactory store = new MemoryStoreFactory();
        LogFactory log = new EmittingLogFactory(new FileLogFactory(settings));
        DefaultMessageFactory messages = new DefaultMessageFactory();
        Connector connector = role.equals("acceptor")
                ? new SocketAcceptor(app, store, settings, log, messages)
                : new SocketInitiator(app, store, settings, log, messages);
        connector.start();
        if (connector instanceof SocketAcceptor acceptor) {
            port = ((InetSocketAddress) acceptor.getEndpoints().iterator().next().getLocalAddress()).getPort();
        }
        emit("ready", Integer.toString(port));

        BufferedReader in = new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
        String line;
        while ((line = in.readLine()) != null) {
            String[] parts = line.split(" ", 2);
            String arg = parts.length > 1 ? parts[1] : "";
            try {
                if (parts[0].equals("quit")) {
                    break;
                }
                run(Session.lookupSession(id), parts[0], arg);
                emit("ok", parts[0]);
            } catch (Exception e) {
                emit("error", parts[0] + ": " + e);
            }
        }
        connector.stop(true);
        System.exit(0);
    }

    private static final char[] STORE_PASSWORD = "turbojet".toCharArray();

    /**
     * QuickFIX/J reads its certificates from key stores, so this writes them from the PEM files
     * given: a key store holding the certificate to present (empty if none, so that QuickFIX/J
     * doesn't fall back to its bundled one) and a trust store holding the CA.
     */
    private static void configureTls(SessionSettings settings, SessionID id, Map<String, String> opts, String role)
            throws Exception {
        Path dir = Files.createDirectories(Path.of(opts.get("log-dir"), "tls"));
        KeyStore keys = KeyStore.getInstance("PKCS12");
        keys.load(null, null);
        if (opts.containsKey("tls-cert")) {
            Certificate[] chain = certificates(Path.of(opts.get("tls-cert")));
            keys.setKeyEntry("peer", privateKey(Path.of(opts.get("tls-key"))), STORE_PASSWORD, chain);
        }
        KeyStore trust = KeyStore.getInstance("PKCS12");
        trust.load(null, null);
        Certificate[] cas = certificates(Path.of(opts.get("tls-ca")));
        for (int i = 0; i < cas.length; i++) {
            trust.setCertificateEntry("ca" + i, cas[i]);
        }
        settings.setString(id, "SocketUseSSL", "Y");
        settings.setString(id, "SocketKeyStore", store(keys, dir.resolve("keys.p12")));
        settings.setString(id, "KeyStoreType", "PKCS12");
        settings.setString(id, "SocketKeyStorePassword", new String(STORE_PASSWORD));
        settings.setString(id, "SocketTrustStore", store(trust, dir.resolve("trust.p12")));
        settings.setString(id, "TrustStoreType", "PKCS12");
        settings.setString(id, "SocketTrustStorePassword", new String(STORE_PASSWORD));
        settings.setString(id, "NeedClientAuth", opts.getOrDefault("need-client-auth", "N"));
        if (role.equals("initiator")) {
            // Check the server's certificate names the host connected to, as Turbojet's client does.
            settings.setString(id, "EndpointIdentificationAlgorithm", "HTTPS");
        }
    }

    private static String store(KeyStore store, Path path) throws Exception {
        try (OutputStream out = Files.newOutputStream(path)) {
            store.store(out, STORE_PASSWORD);
        }
        return path.toString();
    }

    private static Certificate[] certificates(Path pem) throws Exception {
        try (InputStream in = Files.newInputStream(pem)) {
            return CertificateFactory.getInstance("X.509").generateCertificates(in).toArray(new Certificate[0]);
        }
    }

    /** A PKCS#8 key in PEM, as rcgen writes it: ECDSA, or RSA. */
    private static PrivateKey privateKey(Path pem) throws Exception {
        String base64 = Files.readString(pem).replaceAll("-----[A-Z ]+-----", "").replaceAll("\\s", "");
        PKCS8EncodedKeySpec spec = new PKCS8EncodedKeySpec(Base64.getDecoder().decode(base64));
        try {
            return KeyFactory.getInstance("EC").generatePrivate(spec);
        } catch (GeneralSecurityException e) {
            return KeyFactory.getInstance("RSA").generatePrivate(spec);
        }
    }

    private static void run(Session session, String command, String arg) throws Exception {
        switch (command) {
            case "send" -> send(session, build(arg));
            case "send-many" -> {
                int space = arg.indexOf(' ');
                int count = Integer.parseInt(arg.substring(0, space));
                String fields = arg.substring(space + 1);
                for (int i = 0; i < count; i++) {
                    send(session, build(fields.replace("{i}", Integer.toString(i))));
                }
            }
            case "set-next-sender-seq" -> session.setNextSenderMsgSeqNum(Integer.parseInt(arg));
            case "set-next-target-seq" -> {
                awaitProcessed(session);
                session.setNextTargetMsgSeqNum(Integer.parseInt(arg));
            }
            case "sequence-reset" -> {
                // Not atomic: a Heartbeat sent between these two lines would go out with the old
                // numbering. Scenarios using this keep HeartBtInt long enough that none is due.
                send(session, build("35=4|123=N|36=" + arg));
                session.setNextSenderMsgSeqNum(Integer.parseInt(arg));
            }
            case "test-request" -> send(session, build("35=1|112=" + arg));
            case "logout" -> session.logout();
            case "disconnect" -> session.disconnect("disconnect command", false);
            default -> throw new IllegalArgumentException("unknown command");
        }
    }

    /** Logs to {@code inner} and also emits the wire traffic and errors as events. */
    private record EmittingLogFactory(LogFactory inner) implements LogFactory {
        @Override
        public Log create(SessionID id) {
            Log file = inner.create(id);
            return new Log() {
                @Override public void clear() { file.clear(); }
                @Override public void onIncoming(String msg) {
                    file.onIncoming(msg);
                    emit("in", printable(msg));
                }
                @Override public void onOutgoing(String msg) {
                    file.onOutgoing(msg);
                    emit("out", printable(msg));
                }
                @Override public void onEvent(String text) { file.onEvent(text); }
                @Override public void onWarnEvent(String text) { file.onWarnEvent(text); }
                @Override public void onErrorEvent(String text) {
                    file.onErrorEvent(text);
                    emit("qfj_error", text.replace(SOH, '|').replace('\n', ' '));
                }
            };
        }
    }

    /**
     * Waits until QuickFIX/J has finished with the last message it delivered. It advances its
     * expected MsgSeqNum only after fromAdmin or fromApp returns, so changing the number as soon
     * as the event is out could be overwritten by that pending increment.
     */
    private static void awaitProcessed(Session session) throws InterruptedException {
        long deadline = System.nanoTime() + 5_000_000_000L;
        while (session.getExpectedTargetNum() <= lastDelivered) {
            if (System.nanoTime() > deadline) {
                throw new IllegalStateException("still processing MsgSeqNum " + lastDelivered);
            }
            Thread.sleep(1);
        }
    }

    private static void delivered(Message msg) throws FieldNotFound {
        lastDelivered = Math.max(lastDelivered, msg.getHeader().getInt(MsgSeqNum.FIELD));
    }

    private static void send(Session session, Message msg) {
        if (!session.send(msg)) {
            throw new IllegalStateException("not sent: session not logged on");
        }
    }

    /** A message from {@code tag=value|...}; 35 goes in the header, the rest in the body. */
    private static Message build(String fields) {
        Message msg = new Message();
        for (String field : fields.split("\\|")) {
            int eq = field.indexOf('=');
            int tag = Integer.parseInt(field.substring(0, eq));
            // SOH, written \x01, may be sent inside a data field.
            String value = field.substring(eq + 1).replace("\\x01", String.valueOf(SOH));
            if (tag == 35 || tag == 212 || tag == 213) {
                msg.getHeader().setString(tag, value);
            } else {
                msg.setString(tag, value);
            }
        }
        return msg;
    }

    @Override public void onCreate(SessionID id) {}
    @Override public void onLogon(SessionID id) { emit("logon", ""); }
    @Override public void onLogout(SessionID id) { emit("logout", ""); }
    @Override public void toAdmin(Message msg, SessionID id) { emit("to_admin", wire(msg)); }
    @Override public void fromAdmin(Message msg, SessionID id) throws FieldNotFound {
        delivered(msg);
        emit("from_admin", wire(msg));
    }
    @Override public void toApp(Message msg, SessionID id) { emit("to_app", wire(msg)); }
    @Override public void fromApp(Message msg, SessionID id) throws FieldNotFound {
        delivered(msg);
        emit("from_app", wire(msg));
    }
}
