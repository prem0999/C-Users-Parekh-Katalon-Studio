import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.0.0.79/')

WebUI.setText(findTestObject('Page_/input_formUSR_USERNAME'), 'wasm.001')

WebUI.setText(findTestObject('Page_/input_formUSR_PASSWORD_MASK'), 'admin')

WebUI.setText(findTestObject('Page_/input_formUSER_ENV'), 'devtest')

WebUI.click(findTestObject('Page_/input_formBSUBMIT'))

WebUI.click(findTestObject('Page_(wasm.001 in devtest)/span_New case'))

WebUI.doubleClick(findTestObject('Page_(wasm.001 in devtest)/span_PAN Health Customer Creat'))

WebUI.setText(findTestObject('Page_(wasm.001 in devtest)/input_formcust'), '1z')

WebUI.selectOptionByValue(findTestObject('Page_(wasm.001 in devtest)/select_PAN Counter StockistPAN'), 'F956CF095DB74A21A0A6F702026334CB', 
    true)

WebUI.setText(findTestObject('Page_(wasm.001 in devtest)/input_formpan_no'), 'qqqqq1111a')

WebUI.setText(findTestObject('Page_(wasm.001 in devtest)/input_formemail'), 'p@gmail.com')

WebUI.setText(findTestObject('Page_(wasm.001 in devtest)/input_formcontact_person'), 'asd')

WebUI.selectOptionByValue(findTestObject('Page_(wasm.001 in devtest)/select_ST Outward PricelistSup'), 'F9D297B7FA6046659E783C92F29EAB62', 
    true)

WebUI.click(findTestObject('Page_(wasm.001 in devtest)/html_PMDynaform'))

WebUI.setText(findTestObject('Page_(wasm.001 in devtest)/input_formsales_person_label'), 's')

WebUI.click(findTestObject('Page_(wasm.001 in devtest)/a_NRSM001'))

WebUI.closeBrowser()

