package proto;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import okhttp3.HttpUrl;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.logging.HttpLoggingInterceptor;
import proto.data.positions.DateTimeSerializer;
import retrofit2.converter.gson.GsonConverterFactory;
import retrofit2.Retrofit;

import java.time.ZonedDateTime;

public class ServiceGenerator {
    static Retrofit retrofit;
    static Gson gson;

    public ServiceGenerator(HttpUrl url, String token, HttpLoggingInterceptor.Level logLevel) {
        GsonBuilder gsonBuilder = new GsonBuilder();
        gsonBuilder.registerTypeAdapter(ZonedDateTime.class, new DateTimeSerializer());
        gson = gsonBuilder.create();
        GsonConverterFactory gsonConverterFactory = GsonConverterFactory.create(gson);

        retrofit = new Retrofit.Builder()
                .baseUrl(url)
                .client(createDefaultOkHttpClient(token, logLevel))
                .addConverterFactory(gsonConverterFactory)
                .build();
    }

    public CirclesApi makeService() {
        return retrofit.create(CirclesApi.class);
    }

    public OkHttpClient createDefaultOkHttpClient(String token, HttpLoggingInterceptor.Level level) {
        HttpLoggingInterceptor loggingInterceptor = new HttpLoggingInterceptor();
        loggingInterceptor.setLevel(level);

        OkHttpClient.Builder builder = new OkHttpClient().newBuilder()
        // Log requests
            .addInterceptor(loggingInterceptor)

        // Add token header to every request
            .addInterceptor(chain -> {
                Request request = chain.request().newBuilder()
                    .addHeader("Authorization", "Bearer " + token)
                    .build();
                return chain.proceed(request);
            });

        return builder.build();
    }

    public static Retrofit getRetrofit() {
        return retrofit;
    }

    public static Gson getGson() {
        return gson;
    }
}
