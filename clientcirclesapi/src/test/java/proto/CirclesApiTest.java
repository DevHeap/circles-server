package proto;

import okhttp3.HttpUrl;
import okhttp3.logging.HttpLoggingInterceptor;
import org.junit.Before;
import org.junit.Ignore;
import org.junit.Test;
import proto.data.error.ApiError;
import proto.data.EmptyResponse;
import proto.data.positions.PositionUpdate;
import retrofit2.Response;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.ZonedDateTime;

import static org.junit.Assert.*;

public class CirclesApiTest {
    static final int UNAUTHORIZED = 401;
    static final int OK = 200;
    CirclesApi api;


    @Before
    public void init() throws Exception {
        String token = new String(Files.readAllBytes(Paths.get("../examples/firebase_token.txt")), StandardCharsets.UTF_8)
                .trim();
        api = new ServiceGenerator(HttpUrl.parse("http://localhost/"), token, HttpLoggingInterceptor.Level.BASIC)
                .makeService();
    }

    private void printResponse(Response<?> response) throws Exception {
        if(response.errorBody() != null) {
            System.out.println("Error: " + response.errorBody().string());
        }
        if(response.body() != null) {
            System.out.println("Body:  " + response.body());
        }
    }

    @Ignore
    @Test
    public void wrongToken() throws Exception {
        CirclesApi api = new ServiceGenerator(HttpUrl.parse("http://localhost/"), "wrong token", HttpLoggingInterceptor.Level.BASIC)
                .makeService();

        Response<EmptyResponse> response = api.updatePosition(new PositionUpdate()).execute();
        printResponse(response);

        assertFalse(response.isSuccess());
        assertEquals(response.code(), UNAUTHORIZED);

        ApiError error = ApiError.parseError(response);
    }

    @Ignore
    @Test
    public void updatePosition() throws Exception {
        PositionUpdate positionUpdate = new PositionUpdate();
        positionUpdate.setTime(ZonedDateTime.now());
        positionUpdate.setLatitude(55.751716);
        positionUpdate.setLongitude(48.74731);

        Response<EmptyResponse> response = api.updatePosition(positionUpdate).execute();
        printResponse(response);

        assertTrue(response.isSuccess());
        assertEquals(response.code(), OK);
    }

}
