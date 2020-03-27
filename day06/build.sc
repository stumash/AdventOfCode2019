import mill._
import scalalib._

object day06 extends SbtModule {
  override def scalaVersion = "2.13.1"

  override def ivyDeps = Agg(
    ivy"org.scalatest::scalatest:3.1.1",
    ivy"com.lihaoyi::upickle:0.9.5"
  )

  object test extends Tests {
    override def testFrameworks = Seq("org.scalatest.tools.Framework")
  }
}
