Feature: Users: Look up a username to see if it exists

    Scenario Outline: Look up an unknown username: <Username>
        When I look up the username '<Username>'
        Then the username does not exist

        Examples:
            | Username      |
            | unknown       |
            | !@$%^&*()-=_+ |
            | <>?,./:";[]{} |

    Scenario Outline: Look up a known username: <Username>
        # Given a user exists with details:
        # | Username | <Username> |
        When I look up the username '<Username>'
        Then the username does exist

        Examples:
            | Username |
            | known    |
