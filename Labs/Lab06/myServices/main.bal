
import ballerina/http;

service / on new http:Listener(8080){
    resource function get service1(int num) returns int|error {
        http:Client calc2 = check new ("http://localhost:8081");
        int temp = check calc2->get("/service2?num="+num.toString());
        return temp * 2;
        
    }
}

service / on new http:Listener(8081){
    resource function get service2(int num) returns int|error {
        http:Client calc3 = check new ("http://localhost:8082");
        int temp = check calc3->get("/service3?num="+num.toString());
        return temp * 3;
    }
}

service / on new http:Listener(8082){
    resource function get service3(int num) returns int|error {        
        return num * 5;
    }
}
