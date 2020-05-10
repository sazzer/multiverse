Feature: Users: Look up a username to see if it exists

    @wip
    Scenario Outline: Look up an unknown username: <Username>
        When I look up the username '<Username>'
        Then the username does not exist

        Examples:
            | Username |
            | unknown  |
