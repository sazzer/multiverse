Feature: Users: Look up a username to see if it exists

    Scenario Outline: Look up an unknown username: <Username>
        When I look up the username '<Username>'
        Then the username does not exist

        Examples:
            | Username      |
            | unknown       |
            | !@$%^&*()-=_+ |
            | <>?,./:";[]{} |

    # Should really be a Scenario Outline to test different values, but dependant on outstanding issue in Cucumber-Rust
    # https://github.com/bbqsrc/cucumber-rust/issues/49
    Scenario: Look up a known username
        Given a user exists with details:
            | Username | known |
        When I look up the username 'known'
        Then the username does exist
