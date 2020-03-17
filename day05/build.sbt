ThisBuild / scalaVersion := "2.13.1"
ThisBuild / scalacOptions := Seq("-unchecked", "-deprecation")

lazy val main = (project in file("."))
  .settings( name := "Main" )