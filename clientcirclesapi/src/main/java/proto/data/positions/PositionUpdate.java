package proto.data.positions;

import java.time.ZonedDateTime;

// I intentionally DID NOT implement constructors:
// they would be VERY error-prone with that much Floats.
// Exchange values of just two and we have a nasty hard to detect bug on client.
public class PositionUpdate {
    public ZonedDateTime time;
    public Double latitude;
    public Double longitude;
    public Float accuracy;
    public Double altitude;
    public Float vertical_accuracy;
    public Float bearing;
    public Float speed;
    public Float speed_accuracy;
    public String location;

    public ZonedDateTime getTime() {
        return time;
    }

    public Double getLatitude() {
        return latitude;
    }

    public Double getLongitude() {
        return longitude;
    }

    public Float getAccuracy() {
        return accuracy;
    }

    public Double getAltitude() {
        return altitude;
    }

    public Float getVertical_accuracy() {
        return vertical_accuracy;
    }

    public Float getBearing() {
        return bearing;
    }

    public Float getSpeed() {
        return speed;
    }

    public Float getSpeed_accuracy() {
        return speed_accuracy;
    }

    public String getLocation() {
        return location;
    }

    public void setTime(ZonedDateTime time) {
        this.time = time;
    }

    public void setLatitude(Double latitude) {
        this.latitude = latitude;
    }

    public void setLongitude(Double longitude) {
        this.longitude = longitude;
    }

    public void setAccuracy(Float accuracy) {
        this.accuracy = accuracy;
    }

    public void setAltitude(Double altitude) {
        this.altitude = altitude;
    }

    public void setVertical_accuracy(Float vertical_accuracy) {
        this.vertical_accuracy = vertical_accuracy;
    }

    public void setBearing(Float bearing) {
        this.bearing = bearing;
    }

    public void setSpeed(Float speed) {
        this.speed = speed;
    }

    public void setSpeed_accuracy(Float speed_accuracy) {
        this.speed_accuracy = speed_accuracy;
    }

    public void setLocation(String location) {
        this.location = location;
    }
}


