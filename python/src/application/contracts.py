class Command:
    """Базовая команда."""

    pass


class Query:
    """Базовый запрос."""

    pass


class Handler:
    """Обработчик команд и запросов."""

    def handle(self, request):
        raise NotImplementedError


class EmptyCommand(Command):
    """Пример команды без логики."""


class EmptyQuery(Query):
    """Пример запроса без логики."""


class EmptyHandler(Handler):
    """Обработчик-заглушка."""

    def handle(self, request: Command):
        return None
