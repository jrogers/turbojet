import pytest

from turbojet import Message


def test_build_and_read():
    msg = Message("D", [(11, "order-1"), (55, "XYZ")])
    msg[54] = "1"
    assert msg.msg_type == "D"
    assert msg[11] == "order-1"
    assert msg[54] == "1"
    assert msg.get(55) == "XYZ"
    assert msg.get(58) is None


def test_missing_tag_raises_key_error():
    with pytest.raises(KeyError):
        Message("D")[58]


def test_setting_replaces():
    msg = Message("D", [(55, "XYZ")])
    msg[55] = "ABC"
    assert msg[55] == "ABC"


def parties():
    return Message(
        "D",
        [(11, "o1"), (453, "2"), (448, "A"), (447, "D"), (452, "1"), (448, "B"), (452, "3"), (60, "20261017-12:00:00")],
    )


def test_iterate_in_wire_order():
    msg = Message("D", [(11, "o1"), (55, "XYZ")])
    assert list(msg) == [(35, "D"), (11, "o1"), (55, "XYZ")]
    assert len(msg) == 3


def test_group():
    entries = parties().group(453, [448, 447, 452])
    assert [e[448] for e in entries] == ["A", "B"]
    assert entries[0][447] == "D"
    assert entries[1].get(447) is None
    assert entries[1][452] == "3"


def test_group_absent():
    assert Message("D").group(453, [448]) == []


def test_group_count_wrong():
    msg = Message("D", [(453, "3"), (448, "A"), (448, "B")])
    with pytest.raises(ValueError, match="453"):
        msg.group(453, [448])


def test_group_must_start_with_delimiter():
    msg = Message("D", [(453, "1"), (447, "D"), (448, "A")])
    with pytest.raises(ValueError):
        msg.group(453, [448, 447])


def test_nested_group():
    msg = Message("D", [(453, "1"), (448, "A"), (802, "2"), (523, "x"), (803, "1"), (523, "y"), (803, "2")])
    (party,) = msg.group(453, [448, 802, 523, 803])
    assert [s[523] for s in party.group(802, [523, 803])] == ["x", "y"]


def test_data_fields():
    msg = Message("B")
    msg.set_data(95, 96, b"\x01raw")
    assert msg.get_bytes(96) == b"\x01raw"
    assert msg[95] == "4"
    assert msg.get_bytes(58) is None


def test_tags():
    from turbojet import tags

    assert tags.CL_ORD_ID == 11
    assert tags.BEGIN_STRING == 8
    assert Message("D", [(tags.SYMBOL, "XYZ")])[55] == "XYZ"
