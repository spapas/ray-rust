Feature: Tuples Feature
    Scenario: A tuple with w=1.0 is a point
        Given a a tuple(4.3, -4.2, 3.1, 1.0)
            Then a.x = 4.3
            And a.y = -4.2
            And a.z = 3.1
            And a.w = 1.0
            And a is a point
            And a is not a vector

    Scenario: A tuple with w=0 is a vector
        Given a a tuple(4.3, -4.2, 3.1, 0.0)
            Then a.x = 4.3
            And a.y = -4.2
            And a.z = 3.1
            And a.w = 0.0
            And a is not a point
            And a is a vector

    Scenario: Adding two tuples
        Given a1 a tuple(3, -2, 5, 1)
        And a2 a tuple(-2, 3, 1, 0)
            Then a1 + a2 = tuple(1, 1, 6, 1)