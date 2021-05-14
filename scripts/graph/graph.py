"""
Graph the output of `test_event_serialization`.
"""
import os
import json
import math
from pathlib import Path

import matplotlib.pyplot as plt

def plot():
    with open(os.path.join(Path(__file__).parent, "../../target/rw.json")) as f:
        data = json.load(f)

        fig, subplots = plt.subplots(math.ceil(math.sqrt(len(data))), math.floor(math.sqrt(len(data))))

        index = 0

        ylim = max(
            [
                max(i[0] + i[1]) for i in data.values()
            ]
        ) + 2500

        for i in subplots:
            for j in i:
                event_type = list(data.keys())[index]

                read_times = data[event_type][0]
                write_times = data[event_type][1]

                j.plot(read_times)
                j.plot(write_times)

                j.set_ylim([100, ylim])
                j.set_xlabel("Run")
                j.set_ylabel("ns")
                j.legend(["Read", "Write"])
                j.set_title(event_type)

                print(f"[{event_type}] Average Read:", sum(read_times) / len(read_times))
                print(f"[{event_type}] Average Write:", sum(write_times) / len(write_times))

                index += 1
                if index >= len(data):
                    break

        plt.show()

if __name__ == '__main__':
    plot()
