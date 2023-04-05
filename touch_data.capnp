@0xbcbfed68e46d248d;

struct TouchData {
  struct FingerData {
  id @0 :UInt32;
  isPresent @1 :Bool;
  x @2 :Float32;
  y @3 :Float32;
  pressure @4 :Float32;
  size @5 :Float32;
  orientation @6 :Float32;
  touchMajor @7 :Float32;
  touchMinor @8 :Float32;
}

  width @0 :Int32;
  height @1 :Int32;
  fingers @2 :List(FingerData);
}