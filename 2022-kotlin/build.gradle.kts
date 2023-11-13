import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.7.20"
    application
}

repositories {
    mavenCentral()
}

group = "dev.kbraun"
version = "1.0-SNAPSHOT"

subprojects {
    apply(plugin = "kotlin")

    repositories {
        mavenCentral()
    }

    dependencies {
        testImplementation(kotlin("test"))
    }

    tasks.test {
        useJUnitPlatform()
    }

    tasks.withType<KotlinCompile> {
        kotlinOptions.jvmTarget = "1.8"
    }
}

configure(subprojects - project(":lib")) {
    apply(plugin = "application")

    application {
        mainClass.set("MainKt")

        applicationDefaultJvmArgs = listOf("-DinputFile=input.txt")
    }

    task<JavaExec>("example") {
        mainClass.set("MainKt")
        classpath = sourceSets.main.get().runtimeClasspath

        jvmArgs = listOf("-DinputFile=example.txt")
    }

    dependencies {
        implementation(project(":lib"))
    }

}
