<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Current Air Pollution</name>
   <tag></tag>
   <elementGuidId>2d6b02a3-ac55-477f-862c-21f516c9a546</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/air_pollution?lat=-6.261493&amp;lon=106.810600&amp;appid=9fe30a838ec3e88183154e9d6adcfca0</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.base_URL</defaultValue>
      <description></description>
      <id>b1f667de-13d2-4947-a3bb-db4bdfb50684</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//verify status code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

//verify that it's Jakarta Selatan
WS.verifyElementPropertyValue(response, 'x.city.coord.lat', '-6.2615')
WS.verifyElementPropertyValue(response, 'x.city.coord.lon', '106.8106')

//verify parameter
assertThat(response.getResponseText()).contains('co')
assertThat(response.getResponseText()).contains('no')
assertThat(response.getResponseText()).contains('no2')
assertThat(response.getResponseText()).contains('o3')
assertThat(response.getResponseText()).contains('so2')
assertThat(response.getResponseText()).contains('pm2_5')
assertThat(response.getResponseText()).contains('pm10')
assertThat(response.getResponseText()).contains('nh3')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
