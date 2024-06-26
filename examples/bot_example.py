import socket

def init(url, port):
    global s
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        s.connect((url, port))
        print("Connected to server")
    except ConnectionRefusedError:
        print("Connection refused")
        return

def put(x, y):
    byte = "PUT " + str(x) + " " + str(y) + "\n"
    s.send(byte.encode('utf-8'))

def getBoard():
    str = ""
    while not '\n' in str:
        data = s.recv(1024)
        str += data.decode('utf-8')

    str = str.split('\n')[0]
    width = str.split(' ')[2]
    height = str.split(' ')[3]
    boardString = str.split()[4]
    board = [["" for i in range(int(width))] for j in range(int(height))]
    for i in range(int(height)):
        for j in range(int(width)):
            board[i][j] = boardString[i * int(width) + j]
    return board

def getWidth():
    str = ""
    while not '\n' in str:
        data = s.recv(1024)
        str += data.decode('utf-8')
    return str.split(' ')[2]

def getHeight():
    str = ""
    while not '\n' in str:
        data = s.recv(1024)
        str += data.decode('utf-8')
    return str.split(' ')[3]

def getID():
    str = ""
    while not '\n' in str:
        data = s.recv(1024)
        str += data.decode('utf-8')
    return str.split(' ')[1]

def main():

    #connect to the server
    init(url='localhost', port=1312)

    #get the width of the board
    print(getWidth())

    #get the height of the board
    print(getHeight())

    #get your marker in ASCII
    print(getID())

    #print the go board
    print(getBoard())

    #place a piece at 0,0
    put(0, 0)
    print(getBoard())

    s.close()

if __name__ == '__main__':
    main()