package proto.data;

import org.junit.Test;
import proto.data.error.ApiError;
import proto.data.error.ApiErrorException;

public class ApiErrorTest {
    @Test(expected = ApiErrorException.class)
    public void throwException() throws Exception {
        new ApiError("", "").throwException();
    }
}