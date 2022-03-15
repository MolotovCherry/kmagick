apply {
    plugin("org.jetbrains.kotlin.jvm")
}

group = "com.example"
version = ""

dependencies {
    implementation(project(":kmagick"))
}

val jar by tasks.getting(Jar::class) {
    manifest {
        attributes["Main-Class"] = "com.example.cli.MainKt"
    }

    val dependencies = configurations
        .runtimeClasspath
        .get()
        .map(::zipTree) // OR .map { zipTree(it) }
    from(dependencies)
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
}
