import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.6.10"
    id("org.jetbrains.dokka") version "1.6.10"
}

repositories {
    mavenCentral()
}

subprojects {
    apply {
        plugin("org.jetbrains.kotlin.jvm")
        plugin("org.jetbrains.dokka")
    }

    repositories {
        mavenCentral()
    }

    tasks.withType<KotlinCompile> {
        kotlinOptions.jvmTarget = "11"
    }

    // all modules require this dependency
    dependencies {
        implementation("org.objenesis:objenesis:3.2")
    }

    // build a sourcesjar
    tasks {
        val sourcesJar by creating(Jar::class) {
            archiveClassifier.set("sources")
            from(kotlin.sourceSets["main"].kotlin)
        }

        /*val javadoc by getting(Javadoc::class)
        val javadocJar by creating(Jar::class) {
            from(javadoc)
            archiveClassifier.set("javadoc")
        }*/

        artifacts {
            archives(sourcesJar)
            //archives(javadocJar)
        }
    }

    tasks.withType(KotlinCompile::class).all {
        kotlinOptions.freeCompilerArgs += "-opt-in=kotlin.RequiresOptIn"
    }
}
