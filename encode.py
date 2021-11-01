from os import execlp
import os


class QueueEntry:
    """A queue entry that contains the input, output, and crf of the encode to be done."""

    input: str = ''
    output: str = ''
    crf: int = 0

    def __init__(self, input: str, output: str, crf: int) -> None:
        self.input = input
        self.output = output
        self.crf = crf
        pass
    pass

queue: list[QueueEntry] = []

def addFile() -> None:
    """Adds file to the queue as a queue entry."""

    files: list = os.listdir()
    for i, file in enumerate(files):
        print(str(i) + ': ' + file)

    while True:
        try:
            inputFile: int = int(input('Select Input File: '))
            file: str = files[inputFile]
            crf:int  = int(input('CRF: '))
            queue.append(QueueEntry(file, file + '.webm', crf))
            break
        except ValueError:
            print('Invalid crf rating...')
            continue
    pass

def removeFile() -> None:
    """Removes file from the queue."""

    for i, entry in enumerate(queue):
        print(str(i) + ': [INPUT: ' + entry.input + ', OUTPUT: ' + entry.output + ', CRF: ' + str(entry.crf) + ']')
        pass

    while True:
        choice: str = input('Select file to remove: ')
        if choice == 'q':
            return
        
        try:
            choice: int = int(choice)
            queue.pop(choice)
        except ValueError:
            print('Invalid input...')

def changeCRF() -> None:
    """Allows the use to edit and modify the crf of an already present entry"""

    for i, entry in enumerate(queue):
        print(str(i) + ': [INPUT: ' + entry.input + ', OUTPUT: ' + entry.output + ', CRF: ' + str(entry.crf) + ']')
        pass

    while True:
        choice = input('Select file to remove: ')
        if choice == 'q':
            return
        
        try:
            choice: int = int(choice)
            crf: int = int(input("Set new crf: "))
            queue[choice].crf = crf
        except ValueError:
            print('Invalid input...')

def encodeQueue() -> None:
    """If the queue is not empty, this will start ffmpeg and encode each entry one by one."""

    if len(queue) > 0:
        for entry in queue:
            os.system('ffmpeg -hwaccel auto -i "' + entry.input + '" -c:v libaom-av1 -crf ' + str(entry.crf) + ' -b:v 0 -pix_fmt yuv420p -row-mt 1 -tiles 4x4 -cpu-used 8 -c:a libopus -b:a 128K "' + entry.output + '"')
            os.system('cls') # Clear the console for the next encode.
    pass

while True:
    os.system('cls')

    if not queue:
        print('Queue is empty...')
    else:
        for i, entry in enumerate(queue):
            print(str(i) + ': [INPUT: ' + entry.input + ', OUTPUT: ' + entry.output + ', CRF: ' + str(entry.crf) + ']')
            pass
        pass

    try:
        option: str = input('Select Options: [A]dd file; [R]emove File; [C]hange CRF; [E]ncode Query; [Q]uit ').lower()
        if option == 'a':
            addFile()
        elif option == 'r':
            removeFile()
        elif option == 'c':
            changeCRF()
        elif option == 'e':
            encodeQueue()
        elif option == 'q':
            quit(0)
    except ValueError:
        print('Input invalid...')
        continue
    pass