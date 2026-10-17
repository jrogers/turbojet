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
