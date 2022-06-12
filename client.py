from threading import Thread 
import socket 

CONN = ("127.0.0.1", 8080)

def main():
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.connect(CONN)

    Thread(target=sender, args=[server]).start()


    count = 0
    print("Listening")
    while True:
        msg = server.recv(1024)
        if not msg:
            print("EMPTY PACKET")
            break
        print(f"Message received {count}: {msg.decode('utf-8')}")
        count += 1




def sender(sock):
    while True:
        ui = input("SEND ME? ")
        
        if ui == "killme":
            print("Intentionally sending invalid packages...")
            sock.send(b"killmekillmekillme");

        else:
            ui = f"type=msg:msg={ui}"
            print(f"Sending message {ui} to user")
            sock.send(ui.encode("utf-8"))

if __name__ == "__main__":
    main()