package proto;

import proto.data.EmptyJson;
import proto.data.positions.PositionUpdate;
import retrofit2.Call;
import retrofit2.http.Body;
import retrofit2.http.POST;

public interface CirclesApi {
    @POST("/positions")
    Call<EmptyJson> updatePosition(@Body PositionUpdate positionUpdate);
}
