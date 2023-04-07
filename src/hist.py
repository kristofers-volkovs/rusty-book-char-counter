import json
from matplotlib import pyplot as plt


def main():
    file_path = './char_count.json'

    f = open(file_path)
    data = json.load(f)
    f.close()

    _fig, ax = plt.subplots(figsize =(10, 7))

    ticks = [i for i in range(len(data.keys()))]
    ticks_labels = list(data.keys())
    y = list(data.values())

    ax.bar(ticks_labels, y)

    ax.set_xticks(ticks)
    ax.set_xticklabels(ticks_labels)

    plt.tight_layout()
    plt.show()


if __name__ == '__main__':
    main()
