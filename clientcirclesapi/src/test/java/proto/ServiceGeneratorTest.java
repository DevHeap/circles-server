package proto;

import okhttp3.HttpUrl;
import okhttp3.logging.HttpLoggingInterceptor;
import org.junit.Test;

public class ServiceGeneratorTest {
    // Just check that is doesn't fail on creation
    @Test
    public void makeService() throws Exception {
        ServiceGenerator apiFactory = new ServiceGenerator(
                HttpUrl.parse("http://localhost:7701/"),
                "instead_of_token",
                HttpLoggingInterceptor.Level.BASIC
        );

        apiFactory.makeService();
    }
}