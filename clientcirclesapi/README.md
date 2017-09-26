## Cirles RESTful API client library
#### Usage
This library is a mere constructor of [Retrofit2](https://futurestud.io/) service instances.

##### Create a Circles API instances
To construct an Api instances two entities are required:
- base_uri: circles server uri like `http://api.circles.devheap.org/`
- token: Firebase Authentication token

```java
// Firstly, construct a factory with these entities and a prefered Retrofit2 log level:
ServiceGenerator apiFactory = new ServiceGenerator(
    HttpUrl.parse("http://localhost:7701/"),
    firebase_token,
    HttpLoggingInterceptor.Level.BASIC
);

// Then, use it to construct CirclesApi instance
CirclesApi api = apiFactory.makeService();
```

After that you can use this `api` according to [Retrofit2](https://futurestud.io/) docs.

##### Error handling
[Retrofit2](https://futurestud.io/) doesn't support user-defined `error body` so there is a specific type + conversion functions in this library to retrieve server `APIError` containing human-readable Http status code and an error message.

```java
// note: don't do .execute in Android code ;)
Response<EmptyJson> response = api.updatePosition(new PositionUpdate()).execute();
if(!response.isSuccess()) {
    ApiError error = ApiError.parseError(response);
    // Print error message
    System.out.println(error);
    // Or just throw an exception
    error.throwException();
}
```