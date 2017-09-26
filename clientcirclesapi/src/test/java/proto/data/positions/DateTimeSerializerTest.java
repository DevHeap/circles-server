package proto.data.positions;

import com.google.gson.Gson;
import okhttp3.HttpUrl;
import okhttp3.logging.HttpLoggingInterceptor;
import org.junit.Test;
import proto.ServiceGenerator;

import java.time.LocalDateTime;
import java.time.ZoneId;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;

import static org.junit.Assert.*;

public class DateTimeSerializerTest {
    {
        new ServiceGenerator(HttpUrl.parse("http://localhost/"), "token", HttpLoggingInterceptor.Level.BASIC);
    }

    @Test
    public void serialize() throws Exception {
        String time = "2017-09-24T08:41:04";
        LocalDateTime dateTime = LocalDateTime.parse(time, DateTimeFormatter.ISO_LOCAL_DATE_TIME);
        ZonedDateTime zonedDateTime = dateTime.atZone(ZoneId.of("UTC"));
        Gson gson = ServiceGenerator.getGson();
        String json = gson.toJson(zonedDateTime);
        assertEquals("\"" + time + "\"", json);
    }

    @Test
    public void conversionToUtc() throws Exception {
        ZonedDateTime localTime = ZonedDateTime.now(ZoneId.of("Australia/Sydney"));
        ZonedDateTime utcTime = localTime.minusHours(10);

        String utcTimeFormatted = utcTime.format(DateTimeFormatter.ISO_LOCAL_DATE_TIME);

        Gson gson = ServiceGenerator.getGson();
        String json = gson.toJson(localTime);

        assertEquals("\"" + utcTimeFormatted + "\"", json);

    }

}