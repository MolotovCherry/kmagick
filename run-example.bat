@echo off

call gradlew example:jar
java -jar example/build/libs/example.jar
