package proto.data.error;

import okhttp3.ResponseBody;
import proto.ServiceGenerator;
import retrofit2.Converter;
import retrofit2.Response;

import java.io.IOException;
import java.lang.annotation.Annotation;

public class ApiError {
    public String status;
    public String message;

    public ApiError(String status, String message) {
        this.status = status;
        this.message = message;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    public void setMessage(String message) {
        this.message = message;
    }

    public String getStatus() {
        return status;
    }

    public String getMessage() {
        return message;
    }

    public void throwException() {
        throw new ApiErrorException(this);
    }

    public static ApiError parseError(Response<?> response) {
        Converter<ResponseBody, ApiError> converter =
            ServiceGenerator.getRetrofit()
                .responseBodyConverter(ApiError.class, new Annotation[0]);

        ApiError error;
        try {
            error = converter.convert(response.errorBody());
        } catch (IOException e) {
            return new ApiError("Unknown", "Unknown API error");
        }

        return error;
    }

    @Override
    public String toString() {
        return "APIError(" + status + "): " + message;
    }
}
