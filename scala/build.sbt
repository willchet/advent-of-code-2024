ThisBuild / scalaVersion := "3.6.2"
ThisBuild / version := "0.0.1"

lazy val sample = (project in file("."))
  .settings(
    name := "Advent of Code"
  )
