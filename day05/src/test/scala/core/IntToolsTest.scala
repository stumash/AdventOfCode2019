package core

import org.scalatest.matchers.should.Matchers
import org.scalatest.freespec.AnyFreeSpec
import IntTools.IntDigitGetter
import IntTools.toParameterMode
import IntTools.PositionMode
import IntTools.ImmediateMode
import IntTools.ByMode

class IntToolsTest extends AnyFreeSpec with Matchers {
  "IntDigitGetter(i).%@(powerOfTen)" - {
    val i = 123456789
    s"when i is $i and powerOfTen is between 0 and 31 (inclusive)" - {
      "should select the correct digit of i" in {
        (i %@ 12) should be (0)
        (i %@ 11) should be (0)
        (i %@ 10) should be (0)
        (i %@ 9) should be (0)
        (i %@ 8) should be (1)
        (i %@ 7) should be (2)
        (i %@ 6) should be (3)
        (i %@ 5) should be (4)
        (i %@ 4) should be (5)
        (i %@ 3) should be (6)
        (i %@ 2) should be (7)
        (i %@ 1) should be (8)
        (i %@ 0) should be (9)
      }
    }

    "when i is negative number" - {
      "should throw an IllegalArgumentException" in {
        an [IllegalArgumentException] should be thrownBy (-1 %@ 0)
      }
    }

    "when ! 0 <= powerOfTwn <= 30" - {
      "should throw an IllegalArgumentException" in {
        an [IllegalArgumentException] should be thrownBy (1 %@ -1)
        an [IllegalArgumentException] should be thrownBy (1 %@ 31)
      }
    }
  }

  "ParameterMode(i)" - {
    "when i is 0" - {
      "should return PositionMode" in {
        toParameterMode(0) should be(PositionMode)
      }
    }
    "when i is 1" - {
      "should return ImmediateMode" in {
        toParameterMode(1) should be(ImmediateMode)
      }
    }
    "when i != 0 && i != 1" - {
      "should throw IllegalArgumentException" in {
        an [IllegalArgumentException] should be thrownBy (toParameterMode(-1))
        an [IllegalArgumentException] should be thrownBy (toParameterMode(2))
      }
    }
  }

  "ByMode(i).mapIfPos(parameterMode, f: (Int => Int))" - {
    "when i < 0" - {
      "should throw an IllegalArgumentException" in {
        an [IllegalArgumentException] should be thrownBy (ByMode(-1))
      }
    }
    val f: (Int => Int) = _+1
    "when parameterMode is PositionMode" - {
      "should return f(i)" in {
        for (i <- 0 to 10) {
          ByMode(i).mapIfPos(PositionMode, f) should be (f(i))
        }
      }
    }
    "when parameterMode is ImmediateMode" - {
      "should return i" in {
        for (i <- 0 to 10) {
          ByMode(i).mapIfPos(ImmediateMode, f) should be (i)
        }
      }
    }
  }
}
