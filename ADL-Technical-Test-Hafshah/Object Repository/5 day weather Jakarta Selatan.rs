<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5 day weather Jakarta Selatan</name>
   <tag></tag>
   <elementGuidId>13d6e4f2-17f3-421d-ab65-ad7de74682d0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>9fe30a838ec3e88183154e9d6adcfca0</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer 9fe30a838ec3e88183154e9d6adcfca0</value>
      <webElementGuid>8b6b3e0c-01fb-402e-8967-b55ee399b0cc</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/forecast?lat=-6.261493&amp;lon=106.810600&amp;appid=9fe30a838ec3e88183154e9d6adcfca0</restUrl>
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
      <id>97a059cd-cbb4-4735-b4c7-0f050b81b5af</id>
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

//verify success code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

//verify that it's Jakarta Selatan
WS.verifyElementPropertyValue(response, 'x.city.coord.lat', '-6.2615')
WS.verifyElementPropertyValue(response, 'x.city.coord.lon', '106.8106')

//verify parameter is available
assertThat(response.getResponseText()).contains('weather')
assertThat(response.getResponseText()).contains('clouds')
assertThat(response.getResponseText()).contains('wind')
assertThat(response.getResponseText()).contains('temp')
assertThat(response.getResponseText()).contains('feels_like')
assertThat(response.getResponseText()).contains('temp_min')
assertThat(response.getResponseText()).contains('temp_max')
assertThat(response.getResponseText()).contains('pressure')
assertThat(response.getResponseText()).contains('sea_level')
assertThat(response.getResponseText()).contains('grnd_level')
assertThat(response.getResponseText()).contains('humidity')
assertThat(response.getResponseText()).contains('temp_kf')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
