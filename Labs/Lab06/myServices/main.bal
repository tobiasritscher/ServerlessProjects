
import ballerina/http;

service /service1 on new http:Listener(8080){
    resource function get calc1(int num) returns int|error {
        http:Client calc2 = check new ("http://localhost:8081");
        int temp = check calc2->get("/service2/calc2?num="+num.toString());
        return temp * 2;
        
    }
}

service /service2 on new http:Listener(8081){
    resource function get calc2(int num) returns int|error {
        http:Client calc2 = check new ("http://localhost:8082");
        int temp = check calc2->get("/service3/calc3?num="+num.toString());
        return temp*3;
    }
}

service /service3 on new http:Listener(8082){
    resource function get calc3(int num) returns int|error {        
        return num*5;
    }
}