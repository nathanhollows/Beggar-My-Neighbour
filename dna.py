import random

class DNA:

    def __init__(self):
        # Create two hands and shuffle them
        self.hand1 = ['A','J','Q','K', '-','-','-','-','-','-','-','-','-', 'A','J','Q','K', '-','-','-','-','-','-','-','-','-']
        self.hand2 = ['A','J','Q','K', '-','-','-','-','-','-','-','-','-', 'A','J','Q','K', '-','-','-','-','-','-','-','-','-']
        random.shuffle(self.hand1)
        random.shuffle(self.hand2)

        self.init1 = self.hand1.copy()
        self.init2 = self.hand2.copy()

        self.pile = []
        self.tricks = 0
        # Current player (1, -1)
        self.turn = 1
        # Number of cards played
        self.turns = 0
        # How many cards until failure
        self.debt = -1

        # TODO: Formalise how this is going to work
        self.fitness = 0

        self.calcFitness()

    def calcFitness(self):
        hand1 = self.hand1.copy()
        hand2 = self.hand2.copy()
        
        # Play the game
        # While one player hasn't lost or won
        while (len(self.hand1) not in [0, 52] 
                or len(self.hand2) not in [0, 52]):
            self.turns += 1
            if (self.turn == 1):
                hand = self.hand1
                opp = self.hand2
            else:
                hand = self.hand2
                opp = self.hand1

            # Play the top card
            card = hand.pop(0)
            self.pile.append(card)
            # If top card is in court
            if (card in ["J", "Q", "K", "A"]):
                self.turn *= -1
                self.debt = self.getDebt(card)
            # If top card not in court
            else:
                if (self.debt > 0):
                    self.debt -= 1
                elif (self.debt == 0):
                    opp += self.pile
                    self.pile = []
                    self.debt = -1
                    self.tricks += 1

            if (len(hand) == 0): return


        # If the card is an Ace or Court card then set the countdown

        # If the countdown reaches = 0 then player gets the stack


    def getFitness(self):
        return self.turns

    def getDebt(self, card):
        switcher = {
            "J": 1,
            "Q": 2,
            "K": 3,
            "A": 4,
        }

        return switcher.get(card, -1)
