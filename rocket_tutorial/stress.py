import requests

while True:
    r = requests.post('http://localhost:8000')
    print(r.text)
