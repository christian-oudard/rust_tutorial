import requests

while True:
    r = requests.get('http://localhost:8000')
    print(r.text)
