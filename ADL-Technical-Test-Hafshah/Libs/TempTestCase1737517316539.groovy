import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.contribution.WebUiDriverCleaner
import com.kms.katalon.core.mobile.contribution.MobileDriverCleaner
import com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner
import com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner
import com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner


DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner())


RunConfiguration.setExecutionSettingFile('/var/folders/fr/gh11zgqn1h38qrq8vmdpw9440000gn/T/Katalon/20250122_104156/execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runWSVerificationScript(new TestCaseBinding('',[:]), 'import static org.assertj.core.api.Assertions.*\n\nimport com.kms.katalon.core.testobject.RequestObject\nimport com.kms.katalon.core.testobject.ResponseObject\nimport com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS\nimport com.kms.katalon.core.webservice.verification.WSResponseManager\n\nimport groovy.json.JsonSlurper\nimport internal.GlobalVariable as GlobalVariable\n\nRequestObject request = WSResponseManager.getInstance().getCurrentRequest()\n\nResponseObject response = WSResponseManager.getInstance().getCurrentResponse()\n\n//verify success code\nWS.verifyResponseStatusCode(response, 200)\nassertThat(response.getStatusCode()).isEqualTo(200)\n\n//verify that it\'s Jakarta Selatan\nWS.verifyElementPropertyValue(response, \'x.city.coord.lat\', \'-6.2615\')\nWS.verifyElementPropertyValue(response, \'x.city.coord.lon\', \'106.8106\')\n\n//verify parameter is available\nassertThat(response.getResponseText()).contains(\'weather\')\nassertThat(response.getResponseText()).contains(\'clouds\')\nassertThat(response.getResponseText()).contains(\'wind\')\nassertThat(response.getResponseText()).contains(\'temp\')\nassertThat(response.getResponseText()).contains(\'feels_like\')\nassertThat(response.getResponseText()).contains(\'temp_min\')\nassertThat(response.getResponseText()).contains(\'temp_max\')\nassertThat(response.getResponseText()).contains(\'pressure\')\nassertThat(response.getResponseText()).contains(\'sea_level\')\nassertThat(response.getResponseText()).contains(\'grnd_level\')\nassertThat(response.getResponseText()).contains(\'humidity\')\nassertThat(response.getResponseText()).contains(\'temp_kf\')', FailureHandling.STOP_ON_FAILURE, true)

