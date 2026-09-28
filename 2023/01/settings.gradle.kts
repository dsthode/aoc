plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.5.0"
}
rootProject.name = "01"
include("src:main:info.dsthode.aoc.util")
findProject(":src:main:info.dsthode.aoc.util")?.name = "info.dsthode.aoc.util"
