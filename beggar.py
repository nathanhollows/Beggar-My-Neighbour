import dna
import population as pop

def main():
    p = pop.Population(0.01,1000)
    for i in range(p.popmax):
        p.decks[i] = dna.DNA()

    best = p.decks[0]
    for deck in p.decks:
        if deck.turns > best.turns:
            best = deck
    print(best.tricks, best.turns, "".join(best.init1), "/", "".join(best.init2))


if __name__ == "__main__":
    main()
