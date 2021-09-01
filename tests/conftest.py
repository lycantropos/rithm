from hypothesis import (HealthCheck,
                        settings)

settings.register_profile('default',
                          deadline=None,
                          max_examples=10 ** 4,
                          suppress_health_check=[HealthCheck.filter_too_much,
                                                 HealthCheck.too_slow],
                          verbosity=2)
