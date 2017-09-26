package proto.data.error;

public class ApiErrorException extends RuntimeException {
    public ApiErrorException(ApiError error) {
        super(error.toString());
    }
}
