ThisBuild / scalaVersion := "2.13.1"
ThisBuild / scalacOptions := Seq("-unchecked", "-deprecation")

libraryDependencies += "org.scalactic" %% "scalactic" % "3.1.1"
libraryDependencies += "org.scalatest" %% "scalatest" % "3.1.1" % "test"

lazy val main = (project in file("main"))
  .settings( name := "Main" )
  .aggregate(core)
  .dependsOn(core)

lazy val core = (project in file("core"))
  .settings( name := "Core")