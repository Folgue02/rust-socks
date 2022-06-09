from threading import Thread
import socket

BUFF_SIZE = 42069

def main():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(("127.0.0.1", 8080))

    Thread(target=listener, args=[s]).start()

    while True:
        s.sendall(input(">>").encode("utf-8"))



def listener(s):
    while True:
        data = s.recv(BUFF_SIZE)
        if not data:
            print("ERROR")
            break

        else:
            print(data)

if __name__ == "__main__":
    main()
