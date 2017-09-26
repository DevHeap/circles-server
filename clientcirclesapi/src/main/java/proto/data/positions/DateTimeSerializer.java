package proto.data.positions;

import com.google.gson.*;
import java.lang.reflect.Type;
import java.time.ZoneId;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;

public class DateTimeSerializer implements JsonSerializer<ZonedDateTime> {
    static DateTimeFormatter formatter = DateTimeFormatter.ISO_LOCAL_DATE_TIME;
    static ZoneId UTC = ZoneId.of("UTC");
    @Override
    public JsonElement serialize(ZonedDateTime dateTime, Type type, JsonSerializationContext jsonSerializationContext) {
        ZonedDateTime utcDateTime = dateTime.withZoneSameInstant(UTC);
        JsonPrimitive obj = new JsonPrimitive(utcDateTime.format(formatter));
        return obj;
    }
}
