import asyncio
import http.client

HOST = 'localhost:8000'


def increment_counter():
    conn = http.client.HTTPConnection(HOST)
    conn.request('GET', '/')
    r = conn.getresponse()
    assert r.status == 200
    value_before = int(r.read())

    conn = http.client.HTTPConnection(HOST)
    conn.request('POST', '/')
    r = conn.getresponse()
    assert r.status == 200

    conn = http.client.HTTPConnection(HOST)
    conn.request('GET', '/')
    r = conn.getresponse()
    assert r.status == 200
    value_after = int(r.read())

    return (value_before, value_after)


async def main():
    loop = asyncio.get_event_loop()
    results = await asyncio.gather(*[
        loop.run_in_executor(None, increment_counter)
        for _ in range(10)
    ])
    print(results)


if __name__ == '__main__':
    asyncio.run(main())
