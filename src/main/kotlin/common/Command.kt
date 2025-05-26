package com.revtekk.blaze.common

/**
 * Exception thrown when the validation of command arguments fails.
 *
 * This exception is typically used to indicate that input provided for a command
 * does not meet the required criteria or expected format.
 *
 * @constructor Creates an instance of CommandArgumentValidationException with the
 * provided error message describing the validation failure.
 *
 * @param message The detail message explaining the reason for the validation failure.
 */
class CommandArgumentValidationException(message: String): Exception(message)

/**
 * Represents a generic command that can be executed.
 * This is an abstract base class meant to be extended for creating specific commands.
 *
 * @property args The list of arguments required for the command execution.
 */
abstract class Command(protected val args: List<String>) {
    /**
     * Executes the command with the arguments provided during the creation of the instance.
     * Implementations of this method define the specific behavior of the command when it is executed.
     */
    abstract fun execute()
}
