Feature: Healthcheck

    Scenario: Healthcheck
        When I check the health of the system
        Then the system is healthy
        And the component "db" is healthy