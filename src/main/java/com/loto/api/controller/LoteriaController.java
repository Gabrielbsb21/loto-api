package com.loto.api.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("api/loteria")
public class LoteriaController {

    @GetMapping("/teste")
    public String testEndPont() {
        return "Loto-API está funcionando!";
    }
}

