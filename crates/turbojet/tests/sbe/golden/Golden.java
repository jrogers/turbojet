// Writes messages encoded by real-logic's SBE codecs (sbe-tool's Java output), for the Rust codecs'
// tests to decode and to match byte for byte. Run by scripts/sbe-golden.sh. Every field is set,
// optional ones to their null values, since sbe-tool's encoders leave unset fields as zeros.

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.charset.StandardCharsets;

import org.agrona.concurrent.UnsafeBuffer;

public final class Golden {
    public static void main(String[] args) throws Exception {
        Path dir = Path.of(args[0]);
        write(dir.resolve("car.bin"), car());
        write(dir.resolve("b3-new-order-single.bin"), newOrderSingle());
        write(dir.resolve("b3-negotiate.bin"), negotiate());
    }

    static void write(Path path, byte[] bytes) throws Exception {
        Files.write(path, bytes);
    }

    static byte[] car() {
        UnsafeBuffer buffer = new UnsafeBuffer(new byte[4096]);
        baseline.CarEncoder car = new baseline.CarEncoder();
        car.wrapAndApplyHeader(buffer, 0, new baseline.MessageHeaderEncoder())
            .serialNumber(1234)
            .modelYear(2013)
            .available(baseline.BooleanType.T)
            .code(baseline.Model.A)
            .putVehicleCode("abcdef".getBytes(StandardCharsets.US_ASCII), 0);
        for (int i = 0; i < baseline.CarEncoder.someNumbersLength(); i++) {
            car.someNumbers(i, i * 10);
        }
        car.extras().clear().cruiseControl(true).sportsPack(true).sunRoof(false);
        car.engine().capacity(2000).numCylinders((short) 4).putManufacturerCode("123".getBytes(StandardCharsets.US_ASCII), 0)
            .efficiency((byte) 35).boosterEnabled(baseline.BooleanType.T)
            .booster().boostType(baseline.BoostType.NITROUS).horsePower((short) 200);
        car.fuelFiguresCount(3)
            .next().speed(30).mpg(35.9f).usageDescription("Urban Cycle")
            .next().speed(55).mpg(49.0f).usageDescription("Combined Cycle")
            .next().speed(75).mpg(40.0f).usageDescription("Highway Cycle");
        baseline.CarEncoder.PerformanceFiguresEncoder figures = car.performanceFiguresCount(2);
        figures.next().octaneRating((short) 95).accelerationCount(3)
            .next().mph(30).seconds(4.0f).next().mph(60).seconds(7.5f).next().mph(100).seconds(12.2f);
        figures.next().octaneRating((short) 99).accelerationCount(1).next().mph(30).seconds(3.8f);
        car.manufacturer("Honda").model("Civic VTi").activationCode("abcdef");
        return copy(buffer, baseline.MessageHeaderEncoder.ENCODED_LENGTH + car.encodedLength());
    }

    static byte[] newOrderSingle() {
        UnsafeBuffer buffer = new UnsafeBuffer(new byte[4096]);
        b3.entrypoint.fixp.sbe.NewOrderSingleEncoder order = new b3.entrypoint.fixp.sbe.NewOrderSingleEncoder();
        order.wrapAndApplyHeader(buffer, 0, new b3.entrypoint.fixp.sbe.MessageHeaderEncoder())
            .clOrdID(123456789L)
            .securityID(4001L)
            .orderQty(100)
            .account(12345)
            .marketSegmentID((short) 1)
            .side(b3.entrypoint.fixp.sbe.Side.BUY)
            .ordType(b3.entrypoint.fixp.sbe.OrdType.LIMIT)
            .timeInForce(b3.entrypoint.fixp.sbe.TimeInForce.DAY)
            .ordTagID((short) 7)
            .mmProtectionReset(b3.entrypoint.fixp.sbe.Boolean.NULL_VAL)
            .routingInstruction(b3.entrypoint.fixp.sbe.RoutingInstruction.NULL_VAL)
            .selfTradePreventionInstruction(b3.entrypoint.fixp.sbe.SelfTradePreventionInstruction.NULL_VAL)
            .minQty(0)
            .maxFloor(0)
            .investorID(0)
            .expireDate(0)
            .senderLocation("DMA")
            .enteringTrader("TRADR");
        order.price().mantissa(254_500L);
        order.stopPx().mantissa(b3.entrypoint.fixp.sbe.PriceOptionalEncoder.mantissaNullValue());
        order.custodianInfo().custodian(0).custodyAccount(0).custodyAllocationType(0);
        return copy(buffer, b3.entrypoint.fixp.sbe.MessageHeaderEncoder.ENCODED_LENGTH + order.encodedLength());
    }

    static byte[] negotiate() {
        UnsafeBuffer buffer = new UnsafeBuffer(new byte[4096]);
        b3.entrypoint.fixp.sbe.NegotiateEncoder negotiate = new b3.entrypoint.fixp.sbe.NegotiateEncoder();
        negotiate.wrapAndApplyHeader(buffer, 0, new b3.entrypoint.fixp.sbe.MessageHeaderEncoder())
            .sessionID(42)
            .sessionVerID(3)
            .enteringFirm(100)
            .onbehalfFirm(0);
        negotiate.timestamp().time(1_700_000_000_000_000_000L);
        negotiate.credentials("{\"auth_type\":\"basic\",\"username\":\"42\",\"access_key\":\"secret\"}")
            .clientIP("10.0.0.1")
            .clientAppName("turbojet")
            .clientAppVersion("0.2.0");
        return copy(buffer, b3.entrypoint.fixp.sbe.MessageHeaderEncoder.ENCODED_LENGTH + negotiate.encodedLength());
    }

    static byte[] copy(UnsafeBuffer buffer, int length) {
        byte[] out = new byte[length];
        buffer.getBytes(0, out);
        return out;
    }
}
