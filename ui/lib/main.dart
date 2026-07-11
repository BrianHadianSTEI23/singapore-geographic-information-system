import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:http/http.dart' as http;
import 'package:latlong2/latlong.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SG Gender Distribution Heatmap',
      theme: ThemeData.dark(useMaterial3: true),
      home: const MapDashboard(),
      debugShowCheckedModeBanner: false,
    );
  }
}

// Client Side Object representation parsed from Rust Data Stream
class SubzoneModel {
  final String id;
  final String name;
  final int menPopulation;
  final int womenPopulation;
  final List<LatLng> points;

  SubzoneModel({
    required this.id,
    required this.name,
    required this.menPopulation,
    required this.womenPopulation,
    required this.points,
  });

  factory SubzoneModel.fromJson(Map<String, dynamic> json) {
    var coordsJson = json['coordinates'] as List;
    List<LatLng> parsedPoints = coordsJson
        .map((c) => LatLng(c['lat'] as double, c['lng'] as double))
        .toList();

    return SubzoneModel(
      id: json['id'],
      name: json['name'],
      menPopulation: json['men_population'],
      womenPopulation: json['women_population'],
      points: parsedPoints,
    );
  }
}

class MapDashboard extends StatefulWidget {
  const MapDashboard({super.key});

  @override
  State<MapDashboard> createState() => _MapDashboardState();
}

class _MapDashboardState extends State<MapDashboard> {
  String _activeLayer = 'Combined'; // Triggers: 'Men' | 'Women' | 'Combined'
  List<SubzoneModel> _subzones = [];
  bool _isLoading = true;
  String? _hoveredSubzoneInfo;

  @override
  void initState() {
    super.initState();
    _fetchSpatialDemographics();
  }

  // Intercept data from local Rust Pipeline
  Future<void> _fetchSpatialDemographics() async {
    try {
      final response = await http.get(Uri.parse('http://127.0.0.1:8085/api/heatmap'));
      if (response.statusCode == 200) {
        final List<dynamic> data = json.decode(response.body);
        setState(() {
          _subzones = data.map((json) => SubzoneModel.fromJson(json)).toList();
          _isLoading = false;
        });
      }
    } catch (e) {
      debugPrint("Pipeline error fetching data: $e");
      setState(() {
        _isLoading = false;
      });
    }
  }

  // Pure function assigning programmatic Color weights based on active UI Toggles
  Color _calculateChoroplethColor(SubzoneModel subzone) {
    if (_activeLayer == 'Men') {
      double intensity = (subzone.menPopulation / 30000).clamp(0.1, 0.85);
      return Colors.blueAccent.withOpacity(intensity);
    } else if (_activeLayer == 'Women') {
      double intensity = (subzone.womenPopulation / 30000).clamp(0.1, 0.85);
      return Colors.pinkAccent.withOpacity(intensity);
    } else {
      // Metric showing gender ratio skew representation
      double ratio = subzone.menPopulation / (subzone.womenPopulation + 1);
      if (ratio > 1.2) return Colors.cyan.withOpacity(0.65); // Heavy Male Concentration
      if (ratio < 0.8) return Colors.purpleAccent.withOpacity(0.65); // Heavy Female Concentration
      return Colors.amber.withOpacity(0.5); // Balanced demographic parity
    }
  }

  @override
  Widget build(BuildContext context) {
    // Generate Polygon mapping models directly acceptable by FlutterMap canvas
    final polygonLayers = _subzones.map((subzone) {
      return Polygon(
        points: subzone.points,
        color: _calculateChoroplethColor(subzone),
        borderColor: Colors.white60,
        borderStrokeWidth: 1.5,
        isFilled: true,
      );
    }).toList();

    return Scaffold(
      body: Stack(
        children: [
          _isLoading
              ? const Center(child: CircularProgressIndicator())
              : FlutterMap(
                  options: MapOptions(
                    initialCenter: const LatLng(1.3100, 103.8700),
                    initialZoom: 12.0,
                    maxZoom: 15,
                    minZoom: 10,
                  ),
                  children: [
                    TileLayer(
                      urlTemplate: 'https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png',
                      subdomains: const ['a', 'b', 'c', 'd'],
                    ),
                    PolygonLayer(polygons: polygonLayers),
                  ],
                ),

          // Control Layer Selection Overlay UI Card
          Positioned(
            top: 20,
            left: 20,
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.9),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: Colors.white24, width: 1),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Text(
                    'Singapore Gender Matrix',
                    style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    children: ['Men', 'Women', 'Combined'].map((layer) {
                      final isSelected = _activeLayer == layer;
                      return Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 4.0),
                        child: ElevatedButton(
                          style: ElevatedButton.styleFrom(
                            backgroundColor: isSelected ? Colors.teal : Colors.grey[850],
                            foregroundColor: Colors.white,
                          ),
                          onPressed: () {
                            setState(() {
                              _activeLayer = layer;
                            });
                          },
                          child: Text(layer),
                        ),
                      );
                    }).toList(),
                  ),
                ],
              ),
            ),
          ),

          // Interactive Information Box displaying contextual localized payload statistics
          Positioned(
            bottom: 20,
            right: 20,
            child: Container(
              width: 280,
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.85),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: Colors.teal.withOpacity(0.5)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Text('📊 Map Metrics Insight', style: TextStyle(fontWeight: FontWeight.bold, color: Colors.tealAccent)),
                  const Divider(color: Colors.white24),
                  ..._subzones.map((sz) => Padding(
                    padding: const EdgeInsets.symmetric(vertical: 4.0),
                    child: Text(
                      "${sz.name}\n🕺 Men: ${sz.menPopulation} | 💃 Women: ${sz.womenPopulation}",
                      style: const TextStyle(fontSize: 11, color: Colors.white70),
                    ),
                  )),
                ],
              ),
            ),
          )
        ],
      ),
    );
  }
}