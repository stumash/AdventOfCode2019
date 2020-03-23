package core

object IntTools {
  implicit class IntDigitGetter(val i: Int) {
    require(i >= 0)
    def %@(powerOfTen: Int): Int = {
      require(powerOfTen >= 0 && powerOfTen <= 30)
      (i / math.pow(10, powerOfTen).toInt) % 10
    }
  }

  implicit class ByMode(val i: Int) {
    require(i >= 0)

    def mapIfPos(mode: ParameterMode, f: Int => Int): Int = {
      mode match {
        case ImmediateMode => i // immediate mode
        case PositionMode => f(i) // position mode
      }
    }
  }

  implicit def toParameterMode(i: Int): ParameterMode = {
    require(i == 0 || i == 1)

    i match {
      case 0 => PositionMode
      case 1 => ImmediateMode
    }
  }
  sealed trait ParameterMode
  object PositionMode extends ParameterMode
  object ImmediateMode extends ParameterMode
}
