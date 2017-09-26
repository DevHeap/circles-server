package proto;

import proto.data.EmptyResponse;
import proto.data.positions.PositionUpdate;
import retrofit2.Call;
import retrofit2.http.Body;
import retrofit2.http.POST;

public interface CirclesApi {
    @POST("/positions")
    Call<EmptyResponse> updatePosition(@Body PositionUpdate positionUpdate);
}
