class Population:
    popmax = 200;
    mutation = 0.01
    generation = 0
    avgFitnes = 0
    decks = []

    def __init__(self, mutation, popmax):
        self.popmax = popmax
        self.mutation = mutation
        self.decks = [None] * popmax

